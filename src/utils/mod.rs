// [MIT License] Copyright (c) 2024 Michel Novus

use crate::service::{process::RuntimeDir, socket::api::Command};
use serde_json;
use std::{io, io::prelude::*, net::Shutdown, os::unix::net::UnixStream};

/// Envia la señal `EXIT` al socket, para detener el servicio desde el exterior.
pub fn stop_service(runtime_dir: &RuntimeDir) -> io::Result<()> {
    let mut stream = UnixStream::connect(runtime_dir.get_socket().unwrap())?;
    {
        let command = serde_json::to_string(&Command::EXIT)?;
        stream.write_all(command.as_bytes())?;
        stream.shutdown(Shutdown::Write)?;
    }
    {
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer)?;
        stream.shutdown(Shutdown::Read)?;
        println!("{}", buffer);
    }
    Ok(())
}

/// Genera el la unidad servicio de systemd.
pub mod unit_service {
    use crate::service::constants::{
        ARG_START, ARG_STOP, SYSTEMD_BINARY, SYSTEMD_USER_DIRECTORY,
    };
    use std::{fs, io::prelude::*, path::PathBuf, process};
    use users::get_current_username;

    #[derive(Debug)]
    pub enum Error {
        GetUsernameError,
        CreateDirError,
        CreateUnitError,
        UnitAlreadyExistsError,
        SystemdReloadError,
    }

    pub fn generate_unit_service(binary_path: PathBuf) -> Result<(), Error> {
        let current_username = match get_current_username() {
            Some(username) => username.into_string().unwrap(),
            None => return Err(self::Error::GetUsernameError),
        };
        let systemd_user_units_path = PathBuf::from({
            let mut systemd_user_units_path =
                String::from(SYSTEMD_USER_DIRECTORY);
            let range = {
                let first_index =
                    systemd_user_units_path.find("USERNAME").unwrap();
                first_index..first_index + 8
            };
            systemd_user_units_path.replace_range(range, &current_username);
            systemd_user_units_path
        });

        if !systemd_user_units_path.exists() {
            match fs::create_dir_all(&systemd_user_units_path) {
                Ok(()) => (),
                Err(_) => return Err(self::Error::CreateDirError),
            };
        };
        let systemd_unit = {
            let mut systemd_unit = PathBuf::from(systemd_user_units_path);
            systemd_unit.push("localdoc-service.service");
            systemd_unit
        };
        if !systemd_unit.exists() {
            let mut file = match fs::File::create(&systemd_unit) {
                Ok(file) => file,
                Err(_) => return Err(self::Error::CreateUnitError),
            };
            match file.write_all(
                self::generate_unit_file_content(binary_path).as_bytes(),
            ) {
                Ok(()) => (),
                Err(_) => return Err(self::Error::CreateUnitError),
            };
        } else {
            return Err(self::Error::UnitAlreadyExistsError);
        }
        Ok(())
    }

    pub fn reload_systemd_units() -> Result<(), Error> {
        let mut child = match process::Command::new(SYSTEMD_BINARY)
            .arg("--user")
            .arg("daemon-reload")
            .spawn()
        {
            Ok(child) => child,
            Err(_) => return Err(self::Error::SystemdReloadError),
        };
        match &mut child.wait() {
            Ok(_) => Ok(()),
            Err(_) => Err(self::Error::SystemdReloadError),
        }
    }

    fn generate_unit_file_content(binary_path: PathBuf) -> String {
        let mut content = String::new();

        content.push_str("[Unit]\n");
        content.push_str(
            "Description=Unidad de servicio para el programa en \
            segundo plano localdoc-service.service.\n",
        );

        content.push_str("\n[Service]\n");
        content.push_str("Type=forking\n");
        content.push_str(&format!(
            "ExecStart={} {}\n",
            binary_path.to_str().unwrap(),
            ARG_START
        ));
        content.push_str(&format!(
            "ExecStop={} {}\n",
            binary_path.to_str().unwrap(),
            ARG_STOP
        ));

        content.push_str("\n[Install]\n");
        content.push_str("WantedBy=multi-user.target\n");

        content
    }
}

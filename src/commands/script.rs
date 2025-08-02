use crate::{cli::Function, errors::Error, print::Printer, repo::Repository, script::Script};

pub fn script(repo: &Repository, packages: &Vec<String>, function: &Function) {
    for pkg_name in packages {
        let pp = Printer::new(pkg_name.clone());

        let pkg = repo.try_get_package_from_string(pkg_name);

        let Some(pkg) = pkg else {
            pp.warning("Package doesn't exist... skipping");
            continue;
        };

        let path = pkg.install_path().expect("install_path");
        let script = match Script::load(&path, &pp) {
            Ok(s) => s,
            Err(Error::MissingScript) => {
                pp.warning("No .dotman.lua... skipping");
                continue;
            }
            Err(e) => {
                e.print_error();
                continue;
            }
        };

        let err = match function {
            Function::PostUpdate => script.run_postupdate(),
            Function::PostInstall => script.run_postinstall(),
        };

        match err {
            Ok(_) => {}
            Err(e) => {
                e.print_error();
                continue;
            }
        }
    }
}

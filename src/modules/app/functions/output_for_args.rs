use crate::modules::cli::RootArguments;
use crate::modules::output::Output;
use oci_spec::image::Descriptor;

pub fn output_for_args(arguments: &RootArguments, descriptor: &Descriptor) -> Output {
    if arguments.multi_manifest {
        return Output::dir(&arguments.to.join(descriptor.digest().as_ref()));
    }

    if let Some(platform) = descriptor.platform()
        && !arguments.platform.is_empty()
    {
        let path = arguments
            .to
            .join(platform.os().to_string())
            .join(platform.architecture().to_string());

        return Output::dir(path);
    }

    Output::new(&arguments.to)
}

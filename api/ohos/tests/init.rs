
#[cfg(test)]
mod tests{
    extern crate alloc;

    use slint_ohos::platform::OHOSPlatform;


    slint::include_modules!();

    #[test]
    fn ohos_init()  {

        slint::platform::set_platform(Box::new(OHOSPlatform::default()))
            .expect("backend already initialized");
        // slint::ohos::set_platform(Box::<OHOSPlatform>::default()).unwrap();
        let main_window = Demo::new().unwrap();
        main_window.run().unwrap();

        println!("Slint执行成功！");
    }

}
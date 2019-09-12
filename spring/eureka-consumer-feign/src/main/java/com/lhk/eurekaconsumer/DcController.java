package com.lhk.eurekaconsumer;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class DcController {

    @Autowired
    DcClient client;

    @GetMapping("/consumer")
    public String dc() {
        return client.consumer();
    }
}

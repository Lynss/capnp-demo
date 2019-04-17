@0xee8d8123a6363ce5;

struct HelloRequest{
     name@0 :Text;
}

struct HelloResponse{
     message@0 :Text;
}

interface Greeter{
    sayHello@0 (request:HelloRequest) -> (response:HelloResponse);
}
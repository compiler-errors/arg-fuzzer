
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4099(_: S4, _: S1, _: S1, _: S7, _: S2) {}
        
        fn test4099() { foo4099(S3, S7, S4, S7, S5, S7, S3); }
    

        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13053(_: S1, _: S6, _: S3) {}
        
        fn test13053() { foo13053(S0, S2, S2, S6, S2, S1, S1); }
    
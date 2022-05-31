
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo3193(_: S4, _: S6, _: S7) {}
        
        fn test3193() { foo3193(S8, S4, S8, S3, S2); }
    
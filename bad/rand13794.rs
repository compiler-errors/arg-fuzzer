
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13794(_: S3, _: S4, _: S1) {}
        
        fn test13794() { foo13794(S2, S3, S4, S6); }
    
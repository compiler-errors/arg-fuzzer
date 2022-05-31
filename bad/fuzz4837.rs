
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4837(_: S7, _: S8) {}
        
        fn test4837() { foo4837(S3, S7, S3, S6, S5, S5, S5, S5); }
    

        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15023(_: S1, _: S4, _: S1, _: S1) {}
        
        fn test15023() { foo15023(S2, S3, S3, S0, S6, S4, S5); }
    
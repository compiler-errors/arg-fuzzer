
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15967(_: S6, _: S1, _: S0, _: S7, _: S4, _: S3, _: S2) {}
        
        fn test15967() { foo15967(S1, S2, S3, S6, S7, S8); }
    
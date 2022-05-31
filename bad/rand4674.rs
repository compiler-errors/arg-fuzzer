
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4674(_: S6, _: S3, _: S1, _: S5, _: S8) {}
        
        fn test4674() { foo4674(S5, S3, S4, S0, S1, S7, S3); }
    
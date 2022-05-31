
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5515(_: S0, _: S7, _: S2, _: S3, _: S1) {}
        
        fn test5515() { foo5515(S1, S2, S3, S4, S5, S7, S8); }
    
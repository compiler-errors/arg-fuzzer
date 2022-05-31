
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5442(_: S1, _: S4, _: S8) {}
        
        fn test5442() { foo5442(S2, S7, S1, S7, S5, S0); }
    
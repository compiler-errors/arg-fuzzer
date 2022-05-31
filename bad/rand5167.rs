
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5167(_: S2, _: S4, _: S3, _: S3, _: S6) {}
        
        fn test5167() { foo5167(S1, S1, S7, S7, S4, S5, S0); }
    
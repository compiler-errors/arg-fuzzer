
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5035(_: S7, _: S3) {}
        
        fn test5035() { foo5035(S6, S7, S8, S2, S1); }
    
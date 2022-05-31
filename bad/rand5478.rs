
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5478(_: S1, _: S4) {}
        
        fn test5478() { foo5478(S5, S1, S3, S6, S1, S6, S1); }
    
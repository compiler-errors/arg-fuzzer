
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5958(_: S3, _: S4, _: S8) {}
        
        fn test5958() { foo5958(S5, S7, S2, S6, S4, S1); }
    
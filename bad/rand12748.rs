
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12748(_: S1, _: S3) {}
        
        fn test12748() { foo12748(S5, S2, S1, S6, S7); }
    
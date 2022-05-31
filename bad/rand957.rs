
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo957(_: S2, _: S8, _: S1) {}
        
        fn test957() { foo957(S3, S2, S1, S7, S3, S0); }
    
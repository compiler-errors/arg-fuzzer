
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo12079(_: S2, _: S5, _: S6) {}
        
        fn test12079() { foo12079(S7, S1, S6, S3, S2, S4); }
    
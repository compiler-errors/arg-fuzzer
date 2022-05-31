
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8301(_: S6, _: S5, _: S6, _: S1, _: S6, _: S4) {}
        
        fn test8301() { foo8301(S8, S2, S1, S5, S7, S6, S3); }
    
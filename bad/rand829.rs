
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo829(_: S2, _: S3, _: S6, _: S8) {}
        
        fn test829() { foo829(S5, S1, S5, S3, S2, S1, S0); }
    
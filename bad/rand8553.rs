
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8553(_: S4, _: S7) {}
        
        fn test8553() { foo8553(S1, S6, S4, S3, S6, S0); }
    
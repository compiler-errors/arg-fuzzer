
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6553(_: S7, _: S4, _: S0, _: S4) {}
        
        fn test6553() { foo6553(S1, S2, S3, S4, S6, S7, S8); }
    
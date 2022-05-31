
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8224(_: S7, _: S8) {}
        
        fn test8224() { foo8224(S3, S4, S1, S6, S7, S4, S0); }
    
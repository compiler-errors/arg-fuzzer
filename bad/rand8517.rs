
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8517(_: S1, _: S7, _: S8) {}
        
        fn test8517() { foo8517(S2, S1, S5, S0, S5); }
    
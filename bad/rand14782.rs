
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14782(_: S0, _: S3, _: S6, _: S6, _: S7, _: S0) {}
        
        fn test14782() { foo14782(S1, S2, S3, S4, S5, S7, S8); }
    
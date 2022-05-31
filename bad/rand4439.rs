
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4439(_: S7, _: S4, _: S6, _: S1) {}
        
        fn test4439() { foo4439(S2, S5, S2, S6, S0, S7); }
    
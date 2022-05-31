
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4399(_: S6, _: S1, _: S6, _: S0, _: S4, _: S7, _: S3) {}
        
        fn test4399() { foo4399(S3, S4, S7); }
    
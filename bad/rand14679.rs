
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14679(_: S2, _: S3, _: S4, _: S5, _: S6, _: S8) {}
        
        fn test14679() { foo14679(S5, S6, S7, S4, S2, S3, S0); }
    
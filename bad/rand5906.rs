
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5906(_: S6, _: S1, _: S2) {}
        
        fn test5906() { foo5906(S4, S3, S0, S7, S6, S7, S6); }
    
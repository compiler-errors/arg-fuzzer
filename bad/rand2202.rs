
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2202(_: S6, _: S7) {}
        
        fn test2202() { foo2202(S2, S2, S6, S1, S0); }
    
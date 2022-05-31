
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8103(_: S1, _: S7) {}
        
        fn test8103() { foo8103(S4, S5, S0, S1, S2); }
    
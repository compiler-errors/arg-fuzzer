
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1213(_: S7, _: S5, _: S4, _: S7) {}
        
        fn test1213() { foo1213(S6, S7, S4, S7, S0, S0); }
    
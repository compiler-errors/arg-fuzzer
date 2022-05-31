
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2907(_: S5, _: S7, _: S3, _: S4, _: S0, _: S6, _: S3) {}
        
        fn test2907() { foo2907(S1, S2, S3, S4, S5, S6, S7, S8); }
    

        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2037(_: S7, _: S4, _: S5, _: S2, _: S1) {}
        
        fn test2037() { foo2037(S4, S3, S2, S6, S5, S6, S2); }
    
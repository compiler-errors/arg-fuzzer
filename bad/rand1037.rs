
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1037(_: S2, _: S4, _: S6) {}
        
        fn test1037() { foo1037(S1, S5, S5, S7, S6, S2, S2); }
    
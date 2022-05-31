
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2632(_: S2, _: S6) {}
        
        fn test2632() { foo2632(S1, S2, S5, S7, S8); }
    
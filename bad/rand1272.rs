
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1272(_: S3, _: S4, _: S2, _: S7) {}
        
        fn test1272() { foo1272(S5, S3, S5, S7, S5); }
    
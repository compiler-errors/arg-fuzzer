
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1644(_: S3, _: S5, _: S7) {}
        
        fn test1644() { foo1644(S1, S1, S5, S6, S4); }
    
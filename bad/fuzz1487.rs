
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1487(_: S7, _: S1, _: S5) {}
        
        fn test1487() { foo1487(S1, S2, S3, S6, S7, S8); }
    
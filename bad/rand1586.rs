
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1586(_: S3, _: S5, _: S8) {}
        
        fn test1586() { foo1586(S1, S6, S2, S3, S4); }
    
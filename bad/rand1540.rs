
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1540(_: S1, _: S3, _: S4, _: S7) {}
        
        fn test1540() { foo1540(S2, S3, S6, S1, S3, S1); }
    
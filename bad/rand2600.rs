
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2600(_: S4, _: S3, _: S6, _: S2, _: S8) {}
        
        fn test2600() { foo2600(S2, S2, S3, S5, S6, S1); }
    
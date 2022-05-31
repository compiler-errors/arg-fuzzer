
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2263(_: S2, _: S3, _: S4) {}
        
        fn test2263() { foo2263(S1, S3, S3, S2, S1, S3); }
    
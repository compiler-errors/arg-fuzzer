
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2491(_: S4, _: S7, _: S4, _: S3, _: S2) {}
        
        fn test2491() { foo2491(S3, S1, S5, S7, S4, S6, S2, S8); }
    
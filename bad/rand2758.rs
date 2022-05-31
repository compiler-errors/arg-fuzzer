
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2758(_: S3, _: S4, _: S2, _: S5) {}
        
        fn test2758() { foo2758(S1, S3, S4, S5, S7); }
    
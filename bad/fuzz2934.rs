
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2934(_: S1, _: S3, _: S4, _: S5) {}
        
        fn test2934() { foo2934(S8, S2, S2, S1, S6); }
    
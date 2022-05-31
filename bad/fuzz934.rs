
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo934(_: S5, _: S5, _: S2) {}
        
        fn test934() { foo934(S1, S2, S3, S4, S5, S6, S7); }
    
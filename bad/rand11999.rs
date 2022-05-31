
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11999(_: S6, _: S2, _: S5, _: S1, _: S7) {}
        
        fn test11999() { foo11999(S5, S4, S3, S7, S2, S8, S6); }
    
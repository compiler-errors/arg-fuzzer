
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11629(_: S5, _: S3, _: S7, _: S1, _: S6) {}
        
        fn test11629() { foo11629(S3, S6, S4, S5, S6, S6, S2); }
    
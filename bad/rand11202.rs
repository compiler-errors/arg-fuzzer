
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11202(_: S4, _: S6) {}
        
        fn test11202() { foo11202(S2, S4, S1); }
    
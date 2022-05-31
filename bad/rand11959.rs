
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11959(_: S1, _: S4) {}
        
        fn test11959() { foo11959(S7, S1, S8, S5, S3); }
    